import { useEffect, useRef, useState } from "react";
import "./Player.scss";
import { listen } from "@tauri-apps/api/event";

type Props = {
  active?: boolean;
  id: number;
};

type TerminalUpdate = {
  playerId: number;
  content: string;
};

const TERMINAL =
  "> go depth 6 info string NNUE evaluation using nn-5af11540bbfe.nnue enabled info depth 1 seldepth 1 multipv 1 score cp 214 nodes 36 nps 12000 hashfull 0 tbhits 0 time 3 pv c6c5 info depth 2 seldepth 2 multipv 1 score cp 214 nodes 68 nps 22666 hashfull 0 tbhits 0 time 3 pv c6c5 info depth 3 seldepth 2 multipv 1 score cp 226 nodes 107 nps 35666 hashfull 0 tbhits 0 time 3 pv h8h5 info depth 4 seldepth 2 multipv 1 score cp 280 nodes 141 nps 35250 hashfull 0 tbhits 0 time 4 pv h8h5 info depth 5 seldepth 3 multipv 1 score cp 280 nodes 174 nps 43500 hashfull 0 tbhits 0 time 4 pv h8h5 a2a3 info depth 6 seldepth 4 multipv 1 score cp 517 nodes 368 nps 73600 hashfull 0 tbhits 0 time 5 pv c6c5 b4c5 bestmove c6c5 ponder b4c5 ";
function Engine({ active, id }: Props) {
  const [term, setTerm] = useState(false);

  // TERMINAL

  useEffect(() => {
    listen<TerminalUpdate>("update_terminal", (e) => {
      if (e.payload.playerId != id) return;
      let temp = terminalContent;
      temp += e.payload.content;
      setTerminalContent(temp);
    });
  }, []);

  const scrollDummy = useRef<HTMLDivElement>(null);

  const [terminalContent, setTerminalContent] = useState(TERMINAL);

  useEffect(() => {
    scrollDummy.current!.scrollIntoView({ behavior: "smooth" });
  }, [terminalContent]);

  return (
    <div
      className={`player ${active ? "active" : ""} ${
        term ? "show-terminal" : ""
      } bot border-loader`}
    >
      <div className="flex-inner">
        <div className="left">
          <h2 className="name">stockfish</h2>
          <p className="time">time: 01:07.2</p>
        </div>
        <div className="right">
          <a className="link" onClick={() => setTerm(!term)}>
            {term ? "hide terminal" : "show terminal"}
          </a>
          <br />
          <a className="link">bot options</a>
        </div>
      </div>
      <div className="divider">
        <div className="loader"></div>
      </div>
      <div className="terminal">
        <div dangerouslySetInnerHTML={{ __html: terminalContent }}></div>
        <div ref={scrollDummy}></div>
      </div>
    </div>
  );
}

export default Engine;
