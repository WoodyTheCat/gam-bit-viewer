import "./Board.scss";

function Board() {
  return (
    <div className="board">
      {[...Array(8)].map((_, i) => (
        <div className="row" key={i}>
          {[...Array(8)].map((_, j) => (
            <div key={(7 - i) * 8 + j} className="sq">
              {(7 - i) * 8 + j}
            </div>
          ))}
        </div>
      ))}
    </div>
  );
}

export default Board;
