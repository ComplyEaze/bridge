# #1713 Row S row note (2 Nov 2026). Not for merge.
Head: master 3e87d00e (holds #2182, #2262). Build: the green merge-queue run at that head, artifact unsigned-bundle-smoke-windows-latest (sha256:32c8b77e, expires 9 Nov); record bridge_mcp.exe's sha256. Old build: mcp-v0.7.0 windows-x64 .mcpb.
Book: BRIDGE ABANDON LAB alone, TallyPrime 7.2 Silver, licensed (verdict.rs:138 gives the state nowhere else).
Env: BRIDGE_AGENT_ENABLE_WRITES and _MASTER_WRITES true, BRIDGE_TALLY_PORT=9002.
Lost answer: Bridge alone has no reliable way. A read busy before the click refuses, nothing sent (runtime.rs:3869-4176: ten reads or more, each under the 20 s limit). The only gap, last read to send (:4104 to :4234), is local work. So this row uses a relay: relay.py (here, checked on a stand-in only) passes every byte but holds the first Import answer, and Bridge's 20 s limit ends the send (transport lib.rs:95, :490). Tally's write is real. The row tests what follows a lost answer and says nothing about how an answer gets lost.
Steps:
1. `python -I relay.py 9002 9001 OUT`. MARKS, LEDGERS: `Lab Order S` absent.
2. prepare_ledger_creates, then post_ledger_creates; approve by mouse (tools.rs:745). EXPECT after 20 s attempt_recorded true, dispatch reconciliation_required (tools.rs:1577), create.answer CREATED 1.
3. No post_ledger_creates yet: with the ledger present it settles posted_unattributed.
4. Delete the ledger on Tally's screen. MARKS, LEDGERS: name absent; record the mark.
5. post_ledger_creates, same batch_id (:638, :1064). EXPECT confirmed_absent_after_change, reasons not_in_book, mark_moved_since_intent (verdict.rs:749), verification.basis read_after_send_order.
6. New prepare and post, same name. EXPECT the approval box (hold check :714); Decline, nothing sent. Under-lock check not run.
7. Old build, BRIDGE_AGENT_DATA_DIR on a copy of agent-master-ledger.jsonl, post_ledger_creates same batch_id. EXPECT master_ledger_invalid (0.7.0 ledger.rs:1122).
Fails if step 5 says pre_image_master_changed: whether a delete moves another master's ALTERID is not measured.
Keep tool answers, relay.jsonl, marks.
