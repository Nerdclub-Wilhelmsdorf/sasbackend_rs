import http from "k6/http";
import { sleep } from "k6";
const URL = "https://banking.saswdorf.de";
export const options = {
  vus: 600,
  duration: "10s",
};

export default function () {
  http.get(URL);
  sleep(1);
}
