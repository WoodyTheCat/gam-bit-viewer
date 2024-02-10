import "./App.css";
import Toolbar from "./Toolbar";

function App() {
  return (
    <>
      <Toolbar />
      <div className="container">
        <div className="board-container">
          <div className="info-bar">
            <div className="info info-left">
              <h2>
                <span className="text-accent">human</span>'s turn
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
          <div className="board">
            {[...Array(8)].map((_, i) => (
              <div className="row">
                {[...Array(8)].map((_, j) => (
                  <div key={(7 - i) * 8 + j} className="sq">
                    {(7 - i) * 8 + j}
                  </div>
                ))}
              </div>
            ))}
          </div>
        </div>
      </div>
    </>
  );
}

export default App;
