export function setupHotPropertySlider() {
  let currentIndex = 0;

  let totalPages = parseInt(
    document.getElementById("hot-props-total-pages").value
  );
  let hotPropertySliderEl = document.getElementById("hot-properties-slider");
  let nextButtonEl = document.getElementById("hot-props-next-button");
  let prevButtonEl = document.getElementById("hot-props-previous-button");
  let dotsEl = document.getElementById("hot-property-dots");

  let dotChilds = dotsEl.childNodes;

  for (let i = 0; i < dotChilds.length; i++) {
    dotChilds[i].addEventListener("click", (_) => {
      currentIndex = i;
      hotPropertySliderEl.style.transform = `translateX(-${
        currentIndex * 102
      }%)`;
      updateDotsColor(dotChilds, currentIndex);
    });
  }

  nextButtonEl.addEventListener("click", (_) => {
    currentIndex++;

    if (currentIndex > totalPages - 1) {
      currentIndex = 0;
    }

    hotPropertySliderEl.style.transform = `translateX(-${currentIndex * 102}%)`;
    updateDotsColor(dotChilds, currentIndex);
  });

  prevButtonEl.addEventListener("click", (_) => {
    currentIndex--;

    if (currentIndex < 0) {
      currentIndex = totalPages - 1;
    }

    hotPropertySliderEl.style.transform = `translateX(-${currentIndex * 102}%)`;
    updateDotsColor(dotChilds, currentIndex);
  });
}

export function setupPropertyPictureSlider() {
  let hotPropsPictureContainerEl =
    document.querySelectorAll(".picture-container");

  hotPropsPictureContainerEl.forEach((element) => {
    let currentPictureIndex = 0;

    let sliderContainerEl = element.querySelector(".picture-slider-container");

    let hotPropsPictureSliderEl = element.querySelector(".picture-slider");

    let total_picture = parseInt(
      hotPropsPictureSliderEl.querySelector("input").value
    );

    let pictureDots = element.querySelector(".pictures-dots");

    let dotChilds = pictureDots.childNodes;

    for (let i = 0; i < dotChilds.length; i++) {
      dotChilds[i].addEventListener("click", (_) => {
        currentPictureIndex = i;
        hotPropsPictureSliderEl.style.transform = `translateX(-${
          currentPictureIndex * 100
        }%)`;
        updateDotsColor(dotChilds, currentPictureIndex);
      });
    }

    new SwipeContent(sliderContainerEl);

    sliderContainerEl.addEventListener("swipeLeft", function () {
      currentPictureIndex++;

      if (currentPictureIndex > total_picture - 1) {
        currentPictureIndex = 0;
      }

      hotPropsPictureSliderEl.style.transform = `translateX(-${
        currentPictureIndex * 100
      }%)`;

      updateDotsColor(dotChilds, currentPictureIndex);
    });

    sliderContainerEl.addEventListener("swipeRight", function () {
      currentPictureIndex--;

      if (currentPictureIndex < 0) {
        currentPictureIndex = total_picture - 1;
      }

      hotPropsPictureSliderEl.style.transform = `translateX(-${
        currentPictureIndex * 100
      }%)`;

      updateDotsColor(dotChilds, currentPictureIndex);
    });
  });
}

function updateDotsColor(dotChilds, currentPictureIndex) {
  for (let i = 0; i < dotChilds.length; i++) {
    if (i == currentPictureIndex) {
      dotChilds[i].classList.add("bg-blue-500");
      dotChilds[i].classList.remove("bg-blue-200");
    } else {
      dotChilds[i].classList.remove("bg-blue-500");
      dotChilds[i].classList.add("bg-blue-200");
    }
  }
}
