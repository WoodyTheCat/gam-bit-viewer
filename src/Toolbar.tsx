import { invoke } from "@tauri-apps/api/tauri";
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
      <a className="toolbar-btn" onClick={() => invoke("set_board")}>
        set_board
      </a>
    </div>
  );
}

export default Toolbar;
