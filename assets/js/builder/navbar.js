export function showHideTopBarWhenScroll() {
  let prevScrollpos = window.scrollY;
  let navBarEl = document.getElementById("navbar");
  let height = navBarEl.clientHeight;

  window.onscroll = function () {
    let currentScrollPos = window.scrollY;
    if (prevScrollpos > currentScrollPos) {
      navBarEl.style.top = "0";
    } else {
      navBarEl.style.top = `-${height}px`;
    }
    prevScrollpos = currentScrollPos;
  };
}
