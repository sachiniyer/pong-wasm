const canvas = document.getElementById("pongCanvas");
const context = canvas.getContext("2d");

let canvasWidth = window.innerWidth;
let canvasHeight = window.innerHeight;

canvas.width = canvasWidth;
canvas.height = canvasHeight;

const paddleWidth = canvas.width * 0.01;
const paddleHeight = canvas.height * 0.2;
const paddleSpeed = canvas.height * 0.02;
const ballSize = Math.min(canvas.width, canvas.height) * 0.02;
const ballSpeed = Math.min(canvas.width, canvas.height) * 0.005;

let player1Score = 0;
let player2Score = 0;

let player1Y = canvas.height / 2 - paddleHeight / 2;
let player2Y = canvas.height / 2 - paddleHeight / 2;

let ballX = canvas.width / 2;
let ballY = canvas.height / 2;
let ballSpeedX = ballSpeed;
let ballSpeedY = ballSpeed;

function drawPaddles() {
  context.fillStyle = "black";
  context.fillRect(0, player1Y, paddleWidth, paddleHeight);
  context.fillRect(
    canvas.width - paddleWidth,
    player2Y,
    paddleWidth,
    paddleHeight,
  );
}

function drawBall() {
  context.beginPath();
  context.arc(ballX, ballY, ballSize, 0, Math.PI * 2);
  context.fillStyle = "black";
  context.fill();
  context.closePath();
}

function draw() {
  context.clearRect(0, 0, canvas.width, canvas.height);

  drawPaddles();
  drawBall();

  context.font = "30px Arial";
  context.fillText(player1Score, 100, 50);
  context.fillText(player2Score, canvas.width - 100, 50);
}

function update() {
  ballX += ballSpeedX;
  ballY += ballSpeedY;

  handleBallCollisions();

  if (ballX - ballSize <= 0) {
    player2Score++;
    reset();
  }
  if (ballX + ballSize >= canvas.width) {
    player1Score++;
    reset();
  }
}

function handleBallCollisions() {
  if (ballY - ballSize <= 0 || ballY + ballSize >= canvas.height) {
    ballSpeedY = -ballSpeedY;
  }

  if (
    ballX - ballSize <= paddleWidth &&
    ballY >= player1Y &&
    ballY <= player1Y + paddleHeight
  ) {
    ballSpeedX = -ballSpeedX;
    let deltaY = ballY - (player1Y + paddleHeight / 2);
    ballSpeedY = deltaY * 0.2;
  }
  if (
    ballX + ballSize >= canvas.width - paddleWidth &&
    ballY >= player2Y &&
    ballY <= player2Y + paddleHeight
  ) {
    ballSpeedX = -ballSpeedX;
    let deltaY = ballY - (player2Y + paddleHeight / 2);
    ballSpeedY = deltaY * 0.2;
  }
}

function reset() {
  ballX = canvas.width / 2;
  ballY = canvas.height / 2;
}

function gameLoop() {
  update();
  draw();
  requestAnimationFrame(gameLoop);
}

gameLoop();

window.addEventListener("keydown", function (event) {
  switch (event.keyCode) {
    case 38: // Up arrow key
      if (player2Y > 0) {
        player2Y -= paddleSpeed;
      }
      break;
    case 40: // Down arrow key
      if (player2Y < canvas.height - paddleHeight) {
        player2Y += paddleSpeed;
      }
      break;
    case 87: // W key
      if (player1Y > 0) {
        player1Y -= paddleSpeed;
      }
      break;
    case 83: // S key
      if (player1Y < canvas.height - paddleHeight) {
        player1Y += paddleSpeed;
      }
      break;
  }
});

window.addEventListener("resize", function () {
  canvasWidth = window.innerWidth;
  canvasHeight = window.innerHeight;
  canvas.width = canvasWidth;
  canvas.height = canvasHeight;
  player1Y = canvas.height / 2 - paddleHeight / 2;
  player2Y = canvas.height / 2 - paddleHeight / 2;
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
    if (player1Y > 0) {
      player1Y -= paddleSpeed;
    }
  } else if (touchX < halfScreenWidth && touchY >= halfScreenHeight) {
    if (player1Y < canvas.height - paddleHeight) {
      player1Y += paddleSpeed;
    }
  } else if (touchX >= halfScreenWidth && touchY < halfScreenHeight) {
    if (player2Y > 0) {
      player2Y -= paddleSpeed;
    }
  } else {
    if (player2Y < canvas.height - paddleHeight) {
      player2Y += paddleSpeed;
    }
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
    if (player1Y > 0) {
      player1Y -= paddleSpeed;
    }
  } else if (touchX < halfScreenWidth && touchY >= halfScreenHeight) {
    if (player1Y < canvas.height - paddleHeight) {
      player1Y += paddleSpeed;
    }
  } else if (touchX >= halfScreenWidth && touchY < halfScreenHeight) {
    if (player2Y > 0) {
      player2Y -= paddleSpeed;
    }
  } else {
    if (player2Y < canvas.height - paddleHeight) {
      player2Y += paddleSpeed;
    }
  }
});
