(function () {
  if (document.getElementById("pake-nav")) return;

  var nav = document.createElement("div");
  nav.id = "pake-nav";

  var backBtn = document.createElement("button");
  backBtn.textContent = "\u25C0";
  backBtn.title = "Back (Alt+Left)";
  backBtn.addEventListener("click", function () {
    history.back();
  });

  var fwdBtn = document.createElement("button");
  fwdBtn.textContent = "\u25B6";
  fwdBtn.title = "Forward (Alt+Right)";
  fwdBtn.addEventListener("click", function () {
    history.forward();
  });

  var reloadBtn = document.createElement("button");
  reloadBtn.textContent = "\u21BB";
  reloadBtn.title = "Refresh (F5)";
  reloadBtn.addEventListener("click", function () {
    location.reload();
  });

  nav.appendChild(backBtn);
  nav.appendChild(fwdBtn);
  nav.appendChild(reloadBtn);
  if (document.body) {
    document.body.appendChild(nav);
  }

  document.addEventListener("keydown", function (e) {
    if (e.altKey && e.key === "ArrowLeft") {
      e.preventDefault();
      history.back();
    } else if (e.altKey && e.key === "ArrowRight") {
      e.preventDefault();
      history.forward();
    } else if (e.key === "F5" || (e.ctrlKey && e.key === "r")) {
      e.preventDefault();
      location.reload();
    }
  });
})();
