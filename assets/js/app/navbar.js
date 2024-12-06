export function showHideTopBarWhenScroll() {
  const navBarEl = document.getElementById("navbar");

  let prevScrollpos = window.scrollY;

  const handleScroll = () => {
    let currentScrollPos = window.scrollY;
    if (prevScrollpos > currentScrollPos) {
      navBarEl.style.transform = "translateY(0)";
    } else {
      navBarEl.style.transform = `translateY(-100%)`;
    }
    prevScrollpos = currentScrollPos;
  };

  let ticking = false;
  window.addEventListener("scroll", () => {
    if (!ticking) {
      requestAnimationFrame(() => {
        handleScroll();
        ticking = false;
      });
      ticking = true;
    }
  });
}
