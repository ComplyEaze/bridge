//! One stock-by-batch read (#2018): the stock items, then Tally's own Stock Query for each
//! requested item, one at a time in the order asked, inside the bracket the stock summary uses.
//! Each item's answer is held against its own closing in the stock-item collection of the same
//! period ([`tie_stock_query`]); nothing is compared with a voucher.
use super::stock_summary::{
    read_admitted_stock_items, stock_summary_period, AdmittedStockItems, BracketChanged,
    StockSummaryReadError,
};
use super::*;
use bridge_tally_protocol::native_stock_by_batch::{
    godown_subtotals, read_stock_query, tie_stock_query, GodownSubtotal, StockQueryTie,
};
use bridge_tally_protocol::native_stock_query::{
    render_stock_query_request, MeasuredItemName, StockQueryAnswer, StockQueryLine,
};
use bridge_tally_protocol::native_stock_summary::{
    parse_company_inventory_flags, render_company_inventory_flags_request, NativeFlag,
    NativeInventoryFlags, NativeStockItem, StockSummaryAsOf,
};
use bridge_tally_transport::TallyTransportError;

/// The largest Stock Query answer read for one item, in encoded bytes: the transport's limit for
/// that send. A choice, not a measurement: the largest answer captured is 3,152 bytes (one item,
/// two godown lines and its last-purchase groups), and an item's lines grow with the godown and
/// batch pairs that hold stock, which no vouchers read bounds (§12a.23). An answer past it refuses
/// that item as `stock_query_answer_too_large`; the reads go on. PARTIAL: one book, one release.
pub(crate) const STOCK_QUERY_ANSWER_MAX_BYTES: usize = 256 * 1024;

/// What became of one requested item, in the order asked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum StockByBatchItem {
    /// Stock Query answered and the answer was held against the item's closing.
    Read {
        item: NativeStockItem,
        lines: Vec<StockQueryLine>,
        subtotals: Vec<GodownSubtotal>,
        tie: StockQueryTie,
    },
    /// No stock item of the company has this GUID. Nothing was sent for it.
    ItemNotFound { guid: String },
    /// The item's name holds a character no capture showed Stock Query selecting by. Nothing was
    /// sent for it.
    NameUnmeasured { item: NativeStockItem },
    /// Stock Query answered and the answer was refused for this item alone, under `code`.
    Refused {
        item: NativeStockItem,
        code: &'static str,
    },
    /// The send failed or timed out (`code`), so the reads stopped here.
    NotRead {
        item: NativeStockItem,
        code: &'static str,
    },
    /// Not sent: an earlier send failed.
    NotSent { item: NativeStockItem },
}

/// A completed stock-by-batch read. `stopped` holds the code of the send that failed: the reads
/// stopped there, and neither the closing extent nor the brackets after it were read, so nothing
/// shows the book did not move during the items already read.
pub(crate) struct StockByBatchRead {
    pub(crate) from: TallyDate,
    pub(crate) to: TallyDate,
    pub(crate) inventory: NativeInventoryFlags,
    pub(crate) items: Vec<StockByBatchItem>,
    pub(crate) stopped: Option<&'static str>,
    pub(crate) evidence: RuntimeReadEvidence,
}

/// The transport's own code for a send that failed, or a fixed one where it gave none.
fn send_failure_code(error: &anyhow::Error) -> &'static str {
    error
        .chain()
        .find_map(|cause| cause.downcast_ref::<TallyTransportError>())
        .map_or("stock_query_send_failed", TallyTransportError::safe_code)
}

fn too_large(error: &anyhow::Error) -> bool {
    error.chain().any(|cause| {
        matches!(
            cause.downcast_ref::<TallyTransportError>(),
            Some(TallyTransportError::ResponseTooLarge { .. })
        )
    })
}

/// One answer as an item's result: Tally's failure envelope and every parser refusal refuse the
/// item under their own code; otherwise it is held against the closing.
fn read_item(item: NativeStockItem, xml: &str) -> StockByBatchItem {
    let answer = match read_stock_query(xml) {
        Ok(answer) => answer,
        Err(error) => {
            return StockByBatchItem::Refused {
                item,
                code: error.code(),
            }
        }
    };
    let lines = match answer {
        StockQueryAnswer::NoLines => Vec::new(),
        StockQueryAnswer::Lines(ref lines) => lines.clone(),
    };
    match (
        tie_stock_query(&answer, &item.closing.quantity),
        godown_subtotals(&lines),
    ) {
        (Ok(tie), Ok(subtotals)) => StockByBatchItem::Read {
            item,
            lines,
            subtotals,
            tie,
        },
        (Err(error), _) | (_, Err(error)) => StockByBatchItem::Refused {
            item,
            code: error.code(),
        },
    }
}

impl TallyRuntime {
    /// Stock per godown and batch at `as_of` for `guids`, with the queue, mode and identity
    /// brackets of the stock summary: the extent before and after must be equal, or the read
    /// refuses. In that bracket it reads the company's inventory flags and its stock items
    /// (admitted as `stock_summary` admits them), then one Stock Query per requested item that
    /// the items resolve and whose name is measured, in the order asked, then the extent again.
    /// A send that fails stops the reads: the items after it are not sent, and the result says
    /// so rather than refusing the whole read (`stopped`).
    pub(crate) async fn fetch_stock_by_batch(
        &self,
        config: TallyConfig,
        identity: &VerifiedCompanyIdentity,
        as_of: StockSummaryAsOf,
        guids: Vec<String>,
        today: TallyDate,
    ) -> anyhow::Result<StockByBatchRead> {
        let _lease = self.begin_ordinary_read(&config)?;
        let identity = identity.clone();
        self.execute(config, ReadRetryPolicy::SINGLE_ATTEMPT, move |client| {
            let identity = identity.clone();
            let as_of = as_of.clone();
            let guids = guids.clone();
            let today = today.clone();
            async move {
                let mut evidence = RuntimeReadEvidence::empty();
                let result = async {
                    let changed = BracketChanged {
                        collection: PairedReadValidationError::StockByBatchCollection,
                        extent: PairedReadValidationError::StockByBatchExtent,
                    };
                    let (profile, mode_evidence) = observe_read_boundary(&client).await?;
                    evidence = mode_evidence;
                    if profile == DateBoundaryProfile::EducationRestricted {
                        return Err(StockSummaryReadError::EducationUnqualified.into());
                    }
                    bracket_verified_company_identity(&client, &identity).await?;
                    let extent = client.fetch_company_book_extent(&identity).await?;
                    let period =
                        stock_summary_period(profile, &as_of, extent.books_from(), &today)?;

                    let request = render_company_inventory_flags_request(
                        identity.display_name(),
                        identity.company_guid_value(),
                    );
                    let (xml, bytes, hash) = client
                        .fetch_native_report_paired(request.clone())
                        .await?
                        .require_stable(changed.collection)?;
                    evidence = evidence
                        .clone()
                        .combine(RuntimeReadEvidence::paired(&request, hash, bytes));
                    let company_inventory =
                        parse_company_inventory_flags(&xml, identity.company_guid())?;
                    let inventory = company_inventory.flags;
                    if inventory.inventory_on == NativeFlag::No {
                        return Err(StockSummaryReadError::NotEnabled.into());
                    }

                    let AdmittedStockItems { items, premise, .. } = read_admitted_stock_items(
                        &client,
                        &identity,
                        &extent,
                        &period,
                        company_inventory.item_count,
                        changed,
                        &mut evidence,
                    )
                    .await?;
                    // Items whose admission broke are not queried: the closing extent says
                    // whether the book moved, and otherwise the premise refuses.
                    if let Err(error) = premise {
                        if client.fetch_company_book_extent(&identity).await? != extent {
                            return Err(changed.extent.into());
                        }
                        return Err(error.into());
                    }

                    let mut results = Vec::with_capacity(guids.len());
                    let mut stopped = None;
                    for guid in &guids {
                        let Some(item) = items
                            .rows
                            .iter()
                            .find(|row| row.guid.eq_ignore_ascii_case(guid))
                            .cloned()
                        else {
                            results.push(StockByBatchItem::ItemNotFound { guid: guid.clone() });
                            continue;
                        };
                        let Ok(name) = MeasuredItemName::new(&item.name) else {
                            results.push(StockByBatchItem::NameUnmeasured { item });
                            continue;
                        };
                        if stopped.is_some() {
                            results.push(StockByBatchItem::NotSent { item });
                            continue;
                        }
                        let request =
                            render_stock_query_request(identity.display_name(), &name, &period);
                        match client
                            .fetch_stock_query(request.clone(), STOCK_QUERY_ANSWER_MAX_BYTES)
                            .await
                        {
                            Ok((xml, bytes, hash)) => {
                                evidence = evidence
                                    .clone()
                                    .combine(RuntimeReadEvidence::single(&request, hash, bytes));
                                results.push(read_item(item, &xml));
                            }
                            // Tally answered past the limit: that item refuses, and the gateway
                            // is not wedged, so the reads go on.
                            Err(error) if too_large(&error) => {
                                results.push(StockByBatchItem::Refused {
                                    item,
                                    code: "stock_query_answer_too_large",
                                });
                            }
                            // More sends do not help a gateway that failed or timed out (P7).
                            Err(error) => {
                                let code = send_failure_code(&error);
                                stopped = Some(code);
                                results.push(StockByBatchItem::NotRead { item, code });
                            }
                        }
                    }

                    if stopped.is_none() {
                        if client.fetch_company_book_extent(&identity).await? != extent {
                            return Err(changed.extent.into());
                        }
                        bracket_verified_company_identity(&client, &identity).await?;
                        evidence = evidence
                            .clone()
                            .combine(confirm_read_boundary(&client, profile).await?);
                    }
                    Ok(StockByBatchRead {
                        from: period.from().clone(),
                        to: period.to().clone(),
                        inventory,
                        items: results,
                        stopped,
                        evidence: evidence.clone(),
                    })
                }
                .await;
                result.map_err(|error| with_read_evidence(error, evidence))
            }
        })
        .await
    }
}
