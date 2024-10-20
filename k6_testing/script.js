import http from "k6/http";
import { sleep } from "k6";
const URL = "https://banking.saswdorf.de";
const TOKEN = "Bearer W_97xyk8G]]w";
const USERS = [
  { id: "1", pin: "1234" },
  { id: "2", pin: "1235" },
  { id: "3", pin: "1236" },
  { id: "4", pin: "1237" },
  { id: "5", pin: "1238" },
  { id: "zentralbank", pin: "1234" },
];
export const options = {
  vus: 100,
  duration: "30m",
};

export default function () {
  //random Number from 1 to 12
  var randomNum = Math.floor(Math.random() * 12) + 1;
  if (randomNum < 4) {
    transaction();
  }
  if (randomNum < 8) {
    getBalance();
  }
  if (randomNum < 11) {
    get_log();
  }
  if (randomNum == 12) {
    failed_transaction();
  }
  //random from 1 to 10
  sleep(Math.floor(Math.random() * 10) + 1);
}

function transaction() {
  var [user1, user2] = getTwoRandomUsers();
  const payload = JSON.stringify({
    from: user1.id,
    to: user2.id,
    amount: 1,
    pin: user1.pin,
  });
  const params = {
    headers: {
      Authorization: TOKEN,
    },
  };
  var ans = http.post(URL + "/pay", payload, params);
}

function failed_transaction() {
  var [user1, user2] = getTwoRandomUsers();
  const payload = JSON.stringify({
    from: user1.id,
    to: user2.id,
    amount: 1,
    pin: "0000",
  });
  const params = {
    headers: {
      Authorization: TOKEN,
    },
  };
  http.post(URL + "/pay", payload, params);
  http.post(URL + "/pay", payload, params);
  http.post(URL + "/pay", payload, params);
  http.post(URL + "/pay", payload, params);
}

function get_log() {
  var user = getRandomUser();
  const payload = JSON.stringify({
    acc: user.id,
    pin: user.pin,
  });
  const params = {
    headers: {
      Authorization: TOKEN,
    },
  };
  http.get(URL + "/getLogs", payload, params);
}
function getBalance() {
  var user = getRandomUser();
  const payload = JSON.stringify({
    acc1: user.id,
    pin: user.pin,
  });
  const params = {
    headers: {
      Authorization: TOKEN,
    },
  };
  http.get(URL + "/balanceCheck", payload, params);
}

function getRandomUser() {
  return USERS[Math.floor(Math.random() * USERS.length)];
}

function getTwoRandomUsers() {
  const user1 = getRandomUser();
  let user2 = getRandomUser();
  while (user1.id === user2.id) {
    user2 = getRandomUser();
  }
  return [user1, user2];
}
