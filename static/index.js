var images = document.getElementById("images");

function load_images(sightings) {
  sightings.forEach(function (bird, _b) {
    var img = document.createElement("img");
    img.src = "/sightings/" + bird.uuid;
    img.title = bird.species;
    images.appendChild(img);
  });
}

function fetch_sightings() {
  return fetch("/sightings/").then(function (response) {
    if (response.ok) {
      return response.json();
    } else {
      return Promise.reject(response);
    }
  });
}

fetch_sightings().then(sightings => load_images(sightings));