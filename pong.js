const canvas = document.getElementById("pongCanvas");
const context = canvas.getContext("2d");
let canvasWidth = window.innerWidth;
let canvasHeight = window.innerHeight;
canvas.width = canvasWidth;
canvas.height = canvasHeight;

const title = "Pong";

const Direction = {
  UP: "UP",
  DOWN: "DOWN",
  LEFT: "LEFT",
  RIGHT: "RIGHT",
};

let paddleConfig = {
  width: 0.01,
  height: 0.2,
  speed: 0.02,
};

let ballConfig = {
  size: 0.02,
  dx: 0.005,
  dy: 0.005,
};

class Paddle {
  constructor() {
    this.w = canvas.width * paddleConfig.width;
    this.h = canvas.height * paddleConfig.height;
    this.speed = canvas.height * paddleConfig.speed;
  }

  draw(x, y) {
    context.fillStyle = "black";
    context.fillRect(x, y, this.w, this.h);
  }

  reset() {
    this.w = canvas.width * paddleConfig.width;
    this.h = canvas.height * paddleConfig.height;
    this.speed = canvas.height * paddleConfig.speed;
  }
}

class Ball {
  constructor() {
    this.x = canvas.width / 2;
    this.y = canvas.height / 2;
    this.size = Math.min(canvas.width, canvas.height) * ballConfig.size;
    this.dx = Math.min(canvas.width, canvas.height) * ballConfig.dx;
    this.dy = Math.min(canvas.width, canvas.height) * ballConfig.dy;
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

    this.size = Math.min(canvas.width, canvas.height) * ballConfig.size;
    this.dx = Math.min(canvas.width, canvas.height) * ballConfig.dx;
    this.dy = Math.min(canvas.width, canvas.height) * ballConfig.dy;
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
  () => canvas.height / 2 - (canvas.height * paddleConfig.height) / 2,
  () => 100,
);

let p2 = new Player(
  () => canvas.width - canvas.width * paddleConfig.width,
  () => canvas.height / 2 - (canvas.height * paddleConfig.height) / 2,
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
    ball.dy = deltaY * 0.2;
  }
  if (
    ball.x + ball.size >= p2.x &&
    ball.y >= p2.y &&
    ball.y <= p2.y + p2.paddle.h
  ) {
    ball.dx = -ball.dx;
    let deltaY = ball.y - (p2.y + p2.paddle.h / 2);
    ball.dy = deltaY * 0.2;
  }
}

function update() {
  ball.update();

  handleBallCollisions();

  if (ball.x - ball.size <= 0) {
    p2.score++;
    reset();
  }
  if (ball.x + ball.size >= canvas.width) {
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

function gameLoop() {
  update();
  draw();
  requestAnimationFrame(gameLoop);
}

gameLoop();
