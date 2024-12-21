import React, { useState } from "react";
import axios, { Axios } from "axios";
import Task from "./Task";

interface Task {
  id: number,
  title: string,
  body: string
}
const ShowTasks = () => {
  const [tasks, setTasks] = useState<Array<Task>>([]);

  const fetchTasks = async () => {
    let response = await axios.get("http://127.0.0.1:8080/getTasks");
    let newTasks = response.data.map((obj: any) => {
      return {
        id: obj["id"],
        title: obj["title"],
        body: obj["body"]
      } as Task;
    })
    setTasks([...newTasks]);

  }
  return (
    <div>
      <h2>Show Tasks</h2>
      <button onClick={fetchTasks}>submit</button>
      {tasks.map((task) => {
        return <Task title={task.title} body={task.body} />
      })}
    </div>
  );
}

export default ShowTasks;
