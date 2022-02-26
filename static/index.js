var start = 0;
var end = 10;
var images = document.getElementById("images");
var webcam = document.getElementById("webcam");

function play_webcam() {
  webcam.setAttribute("src", "/webcam");
  webcam.onclick = function () {
    pause_webcam();
  };
}

function pause_webcam() {
  webcam.setAttribute("src", "/frame");
  webcam.onclick = function () {
    play_webcam();
  };
}

function clean_images() {
  images.innerHTML = "";
}

function load_images(sightings) {
  sightings.forEach(function (bird, _b) {
    var div = document.createElement("div");

    var h3 = document.createElement("h3");
    h3.innerText = bird.species;
    var img = document.createElement("img");
    img.src = "/sightings/" + bird.uuid;
    img.title = bird.species;
    var remove_button = document.createElement("button");
    remove_button.textContent = "X";
    remove_button.title = "remove"
    remove_button.className = "btn btn-danger"
    remove_button.onclick = function () {
      remove_sighting(bird.uuid);
    };

    div.appendChild(h3);
    div.appendChild(img);
    div.appendChild(remove_button);
    images.appendChild(div);
  });
}

function remove_sighting(uuid) {
  console.log("remove", uuid);
  fetch("/sightings/" + uuid, {
    method: "delete",
  }).then(reload_images);
}

function fetch_sightings() {
  return fetch("/sightings?start=" + start + "&end=" + end).then(function (
    response
  ) {
    if (response.ok) {
      return response.json();
    } else {
      return Promise.reject(response);
    }
  });
}

function reload_images() {
  clean_images();
  return fetch_sightings().then((sightings) => load_images(sightings));
}

function next_images() {
  start = start + 10;
  end = end + 10;
  reload_images();
}

function prev_images() {
  start = start - 10;
  end = end - 10;
  reload_images();
}

reload_images();
setTimeout(pause_webcam, 500);