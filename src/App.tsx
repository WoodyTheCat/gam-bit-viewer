import Toolbar from "./Toolbar";
import Engine from "./Engine";
import Board from "./Board";

function App() {
  return (
    <>
      <Toolbar />
      <div className="container">
        <div className="board-container">
          <div className="info-bar">
            <div className="info info-left">
              <h2>
                <span className="text-accent">your</span> turn
              </h2>
              <p>
                human v bot game <span className="text-tertiary">|</span>{" "}
                <a className="link">pause</a>
              </p>
            </div>
            <div className="info info-right">
              <h2>
                evaluation <span className="text-primary">0</span>
              </h2>
              <p>
                in favour of <b>nobody</b>
              </p>
            </div>
          </div>
          <Board />
          <div className="player-info">
            {/* <Human /> */}
            <Engine id={0} />
            <Engine active id={1} />
          </div>
        </div>
      </div>
    </>
  );
}

export default App;
