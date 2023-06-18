const express = require("express");
const slowDown = require('express-slow-down');
const rateLimit = require('express-rate-limit');
var cors = require('cors');
const app = express();
const port = process.env.PORT || 3001;

app.enable("trust proxy"); // only if you're behind a reverse proxy (Heroku, Bluemix, AWS if you use an ELB, custom Nginx setup, etc)

app.use(cors());

const speedLimiter = slowDown({
  windowMs: 15 * 60 * 1000, // 15 minutes
  delayAfter: 25, // allow 100 requests per 15 minutes, then...
  delayMs: 250 // begin adding 500ms of delay per request above 100:
  // request # 26 is delayed by  250ms
  // request # 27 is delayed by  500ms
  // request # 28 is delayed by 1000ms
  // etc.
});

//  apply to all requests
app.use(speedLimiter);

const limiter = rateLimit({
	windowMs: 15 * 60 * 1000, // 15 minutes
	max: 100, // Limit each IP to 100 requests per `window` (here, per 15 minutes)
	standardHeaders: true, // Return rate limit info in the `RateLimit-*` headers
	legacyHeaders: false, // Disable the `X-RateLimit-*` headers
});

// Apply the rate limiting middleware to all requests
app.use(limiter);

app.get("/", (req, res) => {
    if (req.headers['secret_key'] !== process.env.SECRET_KEY) {
        res.status(401).send("sorry, you are not authorized");
        return;
    }
    res.status(200).send(process.env.SERVER_DATA || '["127.0.0.1", 2402, "password1234"]');
});

app.get("/healthz", (req, res) => {
    res.status(200).send("OK");
});

const server = app.listen(port, () => console.log(`App listening on port ${port}!`));