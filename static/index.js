var start = 0;
var end = 10;
var images = document.getElementById("images");

function clean_images() {
  images.innerHTML = '';
}

function load_images(sightings) {
  sightings.forEach(function (bird, _b) {
    var div = document.createElement("div");

    var h3 = document.createElement("h3");
    h3.innerText = bird.species;
    var img = document.createElement("img");
    img.src = "/sightings/" + bird.uuid;
    img.title = bird.species;

    div.appendChild(h3);
    div.appendChild(img);
    images.appendChild(div);
  });
}

function fetch_sightings() {
  return fetch("/sightings?start=" + start + "&end=" + end).then(function (response) {
    if (response.ok) {
      return response.json();
    } else {
      return Promise.reject(response);
    }
  });
}

function next_images() {
  start = start + 10;
  end = end + 10;
  clean_images();
  return fetch_sightings().then((sightings) => load_images(sightings));
}

function prev_images() {
  start = start - 10;
  end = end - 10;
  clean_images();
  return fetch_sightings().then((sightings) => load_images(sightings));
}

fetch_sightings().then((sightings) => load_images(sightings));
