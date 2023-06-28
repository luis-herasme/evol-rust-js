import "./App.css";
// @ts-ignore
import * as game from "../../pkg";


const app = game.App.new();

// const gameLoop: FrameRequestCallback = (now) => {
//   const dt = now - start;
//   app.update(dt);
//   start = now;
//   requestAnimationFrame(gameLoop);
// };

// gameLoop(start);

setInterval(() => {
  app.update(1000 / 60);
}, 1000 / 60);

function App() {
  return <></>;
}

export default App;
