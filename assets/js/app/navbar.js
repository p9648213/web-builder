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

export function setupMarginNavbar(page) {
  if (page === "search-results") {
    const searchSectionEl = document.getElementById("search-section");
    const navbarEl = document.getElementById("navbar");

    if (searchSectionEl && navbarEl) {
      searchSectionEl.style.marginTop = `${navbarEl.clientHeight}px`;
      searchSectionEl.classList.remove("invisible", "min-h-screen");
    }
  }
  if (page === "property") {
    const propertySectionEl = document.getElementById("property-section");
    const navbarEl = document.getElementById("navbar");

    if (propertySectionEl && navbarEl) {
      propertySectionEl.style.marginTop = `${navbarEl.clientHeight}px`;
      propertySectionEl.classList.remove("invisible", "min-h-screen");
    }
  }
  if (page === "contact") {
    const contactSectionEl = document.getElementById("contact-section");
    const navbarEl = document.getElementById("navbar");

    if (contactSectionEl && navbarEl) {
      contactSectionEl.style.marginTop = `${navbarEl.clientHeight}px`;
      contactSectionEl.classList.remove("invisible", "min-h-screen");
    }
  }
}
