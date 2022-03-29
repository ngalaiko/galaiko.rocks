let angle = 0;
const angleIncrement = (0.5 * Math.PI) / 128;
const TWO_PI = Math.PI * 2;
const offsetBase = 1.55;

async function rotateOffsets(ratio, distanceMultiplier) {
  const main = document.getElementById("content");

  angle += angleIncrement * ratio;
  angle %= TWO_PI;
  main.style.setProperty(
    "--moveX",
    distanceMultiplier * offsetBase * Math.cos(angle) + "px"
  );
  main.style.setProperty(
    "--moveY",
    distanceMultiplier * offsetBase * Math.sin(angle) + "px"
  );
}

//

let rafId;
function update() {
  rafId = requestAnimationFrame((elapsed) => {
    stepper(elapsed);
    update();
  });
}

document.querySelectorAll("main a").forEach((link) => {
  link.dataset["text"] = link.innerText;
  link.style.setProperty("text-decoration", "none");
});

update();

let lastElapsed = 0;
const BASE_DELTA = 1000 / 60; // 1s / 60 frames
let lastDelta = BASE_DELTA;

function stepper(elapsed) {
  lastDelta = elapsed - lastElapsed;
  lastElapsed = elapsed;
  const ratio = lastDelta / BASE_DELTA;
  if (Date.now() - lastMove > 200) {
    mouseTravel = Math.max(0, mouseTravel - ratio * 70);
  }
  rotateOffsets(ratio, Math.max(1, Math.min(mouseTravel / 200, 10)));
}

const lastMousePos = [-1, -1];
let mouseTravel = 0;
let lastMove = 0;
const mouseMoveListener = ({ pageX, pageY }) => {
  if (lastMousePos[0] > -1)
    mouseTravel += Math.max(
      Math.abs(pageX - lastMousePos[0]),
      Math.abs(pageY - lastMousePos[1])
    );
  lastMousePos[0] = pageX;
  lastMousePos[1] = pageY;
  lastMove = Date.now();
};
window.addEventListener("mousemove", mouseMoveListener);
