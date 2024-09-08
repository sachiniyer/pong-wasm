const { startup } = wasm_bindgen;

let worker;

function set_data(key, value) {
  window.localStorage.setItem(key, value);
}

function get_data(key) {
  return window.localStorage.getItem(key);
}

async function run_wasm() {
  await wasm_bindgen();
  console.log("index.js loaded");
  startup();

  worker = new Worker("worker.js");
  worker.onmessage = function (e) {
    if (e.data.type == "setDataMain") {
      set_data(e.data.key, e.data.value);
    }
    if (e.data.type == "getDataMain") {
      worker.postMessage({ type: "getDataWorker", data: get_data(e.data.key) });
    }
  };
}

run_wasm();

const canvas = document.getElementById("pongCanvas");
const context = canvas.getContext("2d");
let canvasWidth = window.innerWidth;
let canvasHeight = window.innerHeight;
canvas.width = canvasWidth;
canvas.height = canvasHeight;

const title = "Pong";

const QUADRANTS = 200;

let widthStep = () => canvas.width / QUADRANTS;
let heightStep = () => canvas.height / QUADRANTS;

const Direction = {
  UP: "UP",
  DOWN: "DOWN",
};

let paddleConfig = {
  width: 1,
  height: 20,
  speed: 3,
};

let ballConfig = {
  size: 2,
  dx: 2.5,
  dy: 2.5,
};

class Paddle {
  constructor() {
    this.w = widthStep() * paddleConfig.width;
    this.h = heightStep() * paddleConfig.height;
    this.speed = heightStep() * paddleConfig.speed;
  }

  draw(x, y) {
    context.fillStyle = "black";
    context.fillRect(x, y, this.w, this.h);
  }

  reset() {
    this.w = widthStep() * paddleConfig.width;
    this.h = heightStep() * paddleConfig.height;
    this.speed = heightStep() * paddleConfig.speed;
  }
}

class Ball {
  constructor() {
    this.x = canvas.width / 2;
    this.y = canvas.height / 2;
    this.size = heightStep() * ballConfig.size;
    this.dx = widthStep() * ballConfig.dx;
    this.dy = (heightStep() * ballConfig.dy) / 2;
  }

  update() {
    this.x += this.dx;
    this.y += this.dy;
  }

  draw() {
    context.beginPath();
    context.arc(this.x, this.y, this.size, 0, Math.PI * 2);
    context.fillStyle = "black";
    context.fill();
    context.closePath();
  }

  reset() {
    this.x = canvas.width / 2;
    this.y = canvas.height / 2;
    this.size = heightStep() * ballConfig.size;
    this.dx = widthStep() * ballConfig.dx;
    this.dy = (heightStep() * ballConfig.dy) / 2;
  }
}

class Player {
  constructor(xf, yf, scoref) {
    this.xf = xf;
    this.yf = yf;
    this.scoref = scoref;
    this.x = xf();
    this.y = yf();
    this.score = 0;
    this.paddle = new Paddle();
  }

  draw() {
    this.paddle.draw(this.x, this.y);
    context.font = "30px Hack";
    context.fillText(this.score, this.scoref(), 50);
  }

  reset() {
    this.x = this.xf();
    this.y = this.yf();
    this.score = this.score;
    this.paddle.reset();
  }
}

let ball = new Ball();

let p1 = new Player(
  () => 0,
  () => canvas.height / 2,
  () => 100,
);

let p2 = new Player(
  () => canvas.width - widthStep() * paddleConfig.width,
  () => canvas.height / 2,
  () => canvas.width - 100,
);

let PlayerEnum = {
  ONE: "ONE",
  TWO: "TWO",
};

function draw() {
  context.clearRect(0, 0, canvas.width, canvas.height);
  p1.draw();
  p2.draw();
  ball.draw();
  context.font = "30px Hack";
  context.fillText(title, canvas.width / 2 - 50, 50);
}

function reset() {
  ball.reset();
}

function handleBallCollisions() {
  if (ball.y - ball.size <= 0 || ball.y + ball.size >= canvas.height) {
    ball.dy = -ball.dy;
  }

  if (
    ball.x - ball.size <= p1.x + p1.paddle.w &&
    ball.y >= p1.y &&
    ball.y <= p1.y + p1.paddle.h
  ) {
    ball.dx = -ball.dx;
    let deltaY = ball.y - (p1.y + p1.paddle.h / 2);
    ball.dy = deltaY * 0.25;
  }
  if (
    ball.x + ball.size >= p2.x &&
    ball.y >= p2.y &&
    ball.y <= p2.y + p2.paddle.h
  ) {
    ball.dx = -ball.dx;
    let deltaY = ball.y - (p2.y + p2.paddle.h / 2);
    ball.dy = deltaY * 0.25;
  }
}

function update() {
  ball.update();

  handleBallCollisions();

  if (ball.x <= 0) {
    p2.score++;
    reset();
  }
  if (ball.x >= canvas.width) {
    p1.score++;
    reset();
  }
}

function movePlayer(player, direction) {
  switch (player) {
    case PlayerEnum.ONE:
      if (direction === Direction.UP) {
        if (p1.y > 0) {
          p1.y -= p1.paddle.speed;
        }
      } else if (direction === Direction.DOWN) {
        if (p1.y < canvas.height - p1.paddle.h) {
          p1.y += p1.paddle.speed;
        }
      }
      break;
    case PlayerEnum.TWO:
      if (direction === Direction.UP) {
        if (p2.y > 0) {
          p2.y -= p2.paddle.speed;
        }
      } else if (direction === Direction.DOWN) {
        if (p2.y < canvas.height - p2.paddle.h) {
          p2.y += p2.paddle.speed;
        }
      }
      break;
  }
}

window.addEventListener("keydown", function (event) {
  switch (event.key) {
    case "w":
      movePlayer(PlayerEnum.ONE, Direction.UP);
      break;
    case "s":
      movePlayer(PlayerEnum.ONE, Direction.DOWN);
      break;
    case "ArrowUp":
      movePlayer(PlayerEnum.TWO, Direction.UP);
      break;
    case "ArrowDown":
      movePlayer(PlayerEnum.TWO, Direction.DOWN);
      break;
  }
});

canvas.addEventListener("touchstart", function (event) {
  event.preventDefault();
  const touchX = event.touches[0].clientX;
  const touchY = event.touches[0].clientY;
  const screenWidth = window.innerWidth;
  const screenHeight = window.innerHeight;
  const halfScreenWidth = screenWidth / 2;
  const halfScreenHeight = screenHeight / 2;

  if (touchX < halfScreenWidth && touchY < halfScreenHeight) {
    movePlayer(PlayerEnum.ONE, Direction.UP);
  } else if (touchX < halfScreenWidth && touchY >= halfScreenHeight) {
    movePlayer(PlayerEnum.ONE, Direction.DOWN);
  } else if (touchX >= halfScreenWidth && touchY < halfScreenHeight) {
    movePlayer(PlayerEnum.TWO, Direction.UP);
  } else {
    movePlayer(PlayerEnum.TWO, Direction.DOWN);
  }
});

canvas.addEventListener("touchmove", function (event) {
  event.preventDefault();
  const touchX = event.touches[0].clientX;
  const touchY = event.touches[0].clientY;
  const screenWidth = window.innerWidth;
  const screenHeight = window.innerHeight;
  const halfScreenWidth = screenWidth / 2;
  const halfScreenHeight = screenHeight / 2;

  if (touchX < halfScreenWidth && touchY < halfScreenHeight) {
    movePlayer(PlayerEnum.ONE, Direction.UP);
  } else if (touchX < halfScreenWidth && touchY >= halfScreenHeight) {
    movePlayer(PlayerEnum.ONE, Direction.DOWN);
  } else if (touchX >= halfScreenWidth && touchY < halfScreenHeight) {
    movePlayer(PlayerEnum.TWO, Direction.UP);
  } else {
    movePlayer(PlayerEnum.TWO, Direction.DOWN);
  }
});

window.addEventListener("resize", function () {
  canvasWidth = window.innerWidth;
  canvasHeight = window.innerHeight;
  canvas.width = canvasWidth;
  canvas.height = canvasHeight;

  p1.reset();
  p2.reset();
  ball.reset();
});

function getGameBoard() {
  let gameBoard = Array(QUADRANTS)
    .fill()
    .map(() => Array(QUADRANTS).fill(false));

  let p1X = Math.floor(p1.x / widthStep());
  let p1Y = Math.floor(p1.y / heightStep());
  let p1W = Math.floor(p1.paddle.w / widthStep());
  let p1H = Math.floor(p1.paddle.h / heightStep());

  let p2X = Math.floor(p2.x / widthStep());
  let p2Y = Math.floor(p2.y / heightStep());
  let p2W = Math.floor(p2.paddle.w / widthStep());
  let p2H = Math.floor(p2.paddle.h / heightStep());

  let ballX = Math.floor(ball.x / widthStep());
  let ballY = Math.floor(ball.y / heightStep());
  let ballSize = Math.floor(ball.size / heightStep());

  for (let i = 0; i < QUADRANTS; i++) {
    for (let j = 0; j < QUADRANTS; j++) {
      if (i >= p1X && i <= p1X + p1W && j >= p1Y && j <= p1Y + p1H) {
        gameBoard[i][j] = true;
      } else if (i >= p2X && i <= p2X + p2W && j >= p2Y && j <= p2Y + p2H) {
        gameBoard[i][j] = true;
      } else if (
        i >= ballX - ballSize &&
        i <= ballX + ballSize &&
        j >= ballY - ballSize &&
        j <= ballY + ballSize
      ) {
        gameBoard[i][j] = true;
      }
    }
  }
  return gameBoard;
}

function gameLoop() {
  update();
  draw();
  if (worker) {
    // worker.postMessage({ topic: "state", data: getGameBoard().flat() });
  }
  setTimeout(gameLoop, 50);
}

gameLoop();
