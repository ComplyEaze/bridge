// A question opens when a link points at it, and opening one puts its address in the address bar, so a
// question can be sent to a partner or a client. The page reads the same with this script absent.
(function () {
  var items = Array.prototype.slice.call(document.querySelectorAll("details.faq-item"));

  function openFromHash() {
    var id = decodeURIComponent((location.hash || "").slice(1));
    if (!id) return;
    var target = document.getElementById(id);
    var item = target && target.closest ? target.closest("details.faq-item") : null;
    if (item) {
      item.open = true;
      item.scrollIntoView();
    }
  }

  items.forEach(function (item) {
    var summary = item.querySelector("summary");
    if (!summary) return;
    summary.addEventListener("click", function () {
      // the click has not yet flipped the state when it fires
      setTimeout(function () {
        if (!item.open || !history.replaceState) return;
        try { history.replaceState(null, "", "#" + item.id); } catch (e) { /* the address stays as it was */ }
      }, 0);
    });
  });

  // one control opens every answer, so a page can be searched, read through or printed whole
  var openAll = document.getElementById("faqOpenAll");
  function setAll(open) { items.forEach(function (item) { item.open = open; }); }
  if (openAll) {
    openAll.hidden = false;
    openAll.addEventListener("click", function () {
      var anyClosed = items.some(function (item) { return !item.open; });
      setAll(anyClosed);
      openAll.textContent = anyClosed ? "Close all answers" : "Open all answers";
    });
  }
  window.addEventListener("beforeprint", function () { setAll(true); });

  openFromHash();
  window.addEventListener("hashchange", openFromHash);
})();
