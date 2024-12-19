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

export function setupHotPropertyPictureSlider() {
  let hotPropsPictureContainerEl = document.querySelectorAll(
    ".hot-props-picture-container"
  );

  hotPropsPictureContainerEl.forEach((element) => {
    let currentPictureIndex = 0;

    let sliderContainerEl = element.querySelector(
      ".hot-props-picture-slider-container"
    );

    let hotPropsPictureSliderEl = element.querySelector(
      ".hot-props-picture-slider"
    );

    let total_picture = parseInt(
      hotPropsPictureSliderEl.querySelector("input").value
    );

    let pictureDots = element.querySelector(".hot-props-pictures-dots");

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

    SwipeListener(sliderContainerEl);

    sliderContainerEl.addEventListener("swipe", (e) => {
      let directions = e.detail.directions;

      if (directions.left) {
        currentPictureIndex++;

        if (currentPictureIndex > total_picture - 1) {
          currentPictureIndex = 0;
        }
        
        hotPropsPictureSliderEl.style.transform = `translateX(-${
          currentPictureIndex * 100
        }%)`;

        updateDotsColor(dotChilds, currentPictureIndex);
      }

      if (directions.right) {
        currentPictureIndex--;

        if (currentPictureIndex < 0) {
          currentPictureIndex = total_picture - 1;
        }

        hotPropsPictureSliderEl.style.transform = `translateX(-${
          currentPictureIndex * 100
        }%)`;

        updateDotsColor(dotChilds, currentPictureIndex);
      }
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
