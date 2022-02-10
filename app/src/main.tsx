import React from "react";
import ReactDOM from "react-dom";
import "./index.css";
import App from "./App";

(BigInt.prototype as any).toJSON = function () {
  return this.toString();
};

document.addEventListener("gesturestart", (e) => e.preventDefault());
document.addEventListener("gesturechange", (e) => e.preventDefault());

ReactDOM.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
  document.getElementById("root")
);
