import "./Toolbar.scss";

function Toolbar() {
  return (
    <div className="toolbar">
      <p className="text-primary">
        <b>gam-bit</b>
      </p>
      <a className="toolbar-btn">file</a>
      <a className="toolbar-btn">game</a>
      <a className="toolbar-btn">prefs</a>
    </div>
  );
}

export default Toolbar;
