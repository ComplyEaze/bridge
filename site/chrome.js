/* ComplyEaze Bridge shared chrome: header scroll state, the mobile menu
   toggle, and the progressive GitHub star count. No scroll listener: the
   header watches a 1px sentinel with IntersectionObserver instead. */
(function () {
  "use strict";

  var header = document.getElementById("ceHeader");
  var sentinel = document.getElementById("ceHeaderSentinel");
  if (header && sentinel && "IntersectionObserver" in window) {
    var io = new IntersectionObserver(
      function (entries) {
        entries.forEach(function (entry) {
          header.classList.toggle("is-scrolled", !entry.isIntersecting);
        });
      },
      { threshold: 0 }
    );
    io.observe(sentinel);
  }

  var toggle = document.getElementById("ceMenuToggle");
  var nav = document.getElementById("ceNav");
  // Below 480px the header star is hidden and GitHub lives in the menu. A page
  // that has not been given the item in its markup gets it here.
  if (nav && !nav.querySelector(".ce-nav__github")) {
    var gh = document.createElement("a");
    gh.className = "ce-nav__github";
    gh.href = "https://github.com/ComplyEaze/bridge";
    gh.target = "_blank";
    gh.rel = "noopener noreferrer";
    gh.textContent = "GitHub";
    nav.appendChild(gh);
  }
  if (toggle && nav) {
    var closeMenu = function () {
      nav.classList.remove("ce-nav--open");
      toggle.setAttribute("aria-expanded", "false");
    };
    toggle.addEventListener("click", function () {
      var open = nav.classList.toggle("ce-nav--open");
      toggle.setAttribute("aria-expanded", String(open));
      // the menu sits before its button in the page, so Tab would skip it: move into it
      if (open) {
        var first = nav.querySelector("a");
        if (first) first.focus();
      }
    });
    Array.prototype.forEach.call(nav.querySelectorAll("a"), function (a) {
      a.addEventListener("click", closeMenu);
    });
    document.addEventListener("keydown", function (event) {
      if (event.key === "Escape" && nav.classList.contains("ce-nav--open")) {
        closeMenu();
        toggle.focus();
      }
    });
    document.addEventListener("click", function (event) {
      if (
        nav.classList.contains("ce-nav--open") &&
        !nav.contains(event.target) &&
        !toggle.contains(event.target)
      ) {
        closeMenu();
      }
    });
  }

  /* The colour world. Cobalt is the page as written (no attribute); red is
     <html data-theme="red">, set before first paint by the one-line script in
     each page's head from the stored choice. The switch is built here, so a
     page without JavaScript shows cobalt and no control that cannot work.
     Storage may be unavailable (private windows, blocked site data): the
     switch still works for the page in view, and the next page starts cobalt. */
  var root = document.documentElement;
  var icon = document.querySelector('link[rel="icon"]');
  function currentTheme() {
    return root.getAttribute("data-theme") === "red" ? "red" : "cobalt";
  }
  function applyTheme(name) {
    if (name === "red") root.setAttribute("data-theme", "red");
    else root.removeAttribute("data-theme");
    if (icon) icon.setAttribute("href", name === "red" ? "brand/favicon-red.svg" : "brand/favicon.svg");
  }
  function storedTheme() {
    try {
      return window.localStorage.getItem("ce-theme") === "red" ? "red" : "cobalt";
    } catch (e) {
      return currentTheme();
    }
  }
  var actions = document.querySelector(".ce-header__actions");
  if (actions) {
    var group = document.createElement("div");
    group.className = "ce-theme";
    group.setAttribute("role", "group");
    group.setAttribute("aria-label", "Colour theme");
    var choose = function (name, store) {
      if (currentTheme() === name) return;
      applyTheme(name);
      if (store) {
        try {
          window.localStorage.setItem("ce-theme", name);
        } catch (e) {
          /* the choice holds for this page only */
        }
      }
      sync();
      document.dispatchEvent(new CustomEvent("ce:theme", { detail: name }));
    };
    var buttons = ["cobalt", "red"].map(function (name) {
      var b = document.createElement("button");
      b.type = "button";
      b.className = "ce-theme__opt ce-theme__opt--" + name;
      var label = name === "red" ? "Red theme" : "Cobalt theme";
      b.setAttribute("aria-label", label);
      b.title = label;
      b.addEventListener("click", function () {
        choose(name, true);
      });
      group.appendChild(b);
      return b;
    });
    var sync = function () {
      var now = currentTheme();
      buttons.forEach(function (b, i) {
        b.setAttribute("aria-pressed", String((i === 1 ? "red" : "cobalt") === now));
      });
    };
    sync();
    actions.insertBefore(group, actions.firstChild);
    // a page restored from the back/forward cache, or another tab changing the choice, follows the stored world
    window.addEventListener("pageshow", function (event) {
      if (event.persisted) choose(storedTheme(), false);
    });
    window.addEventListener("storage", function (event) {
      if (event.key === "ce-theme") choose(storedTheme(), false);
    });
  }

  /* Nothing here contacts GitHub: only the Download and Releases pages do. The header's GitHub
     link shows the word, not a star count. */
})();
