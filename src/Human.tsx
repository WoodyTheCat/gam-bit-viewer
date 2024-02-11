import "./Player.scss";

type Props = {
  active?: boolean;
};

function Human({ active }: Props) {
  return (
    <div className={`player ${active ? "active" : ""} human`}>
      <div className="flex-inner">
        <div className="left">
          <h2 className="name">human</h2>
          <p className="time">time: 01:07.2</p>
        </div>
        <div className="right">
          <a className="link">player options</a>
        </div>
      </div>
    </div>
  );
}

export default Human;
