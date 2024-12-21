import React from "react";
import './Task.css';

interface TaskProps {
  title: string,
  body: string
}
const Task = (props: TaskProps) => {
  return (
    <div className="task">
      <h2>
        {props.title}
      </h2>
      <h3>
        {props.body}
      </h3>
    </div >
  )
}

export default Task;
