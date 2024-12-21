import React, { ChangeEvent, useState } from "react";
import "./TaskForm.css";
import axios, { Axios } from "axios";

const TaskForm = (): React.ReactElement => {

  interface FormData {
    title: string,
    body: string
  }

  const [formData, setFormData] = useState<FormData>({ title: "", body: "" });

  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = event.target;
    setFormData((prevData) => ({
      ...prevData,
      [name]: value,
    }));
  };

  const sendPost = () => axios.post("http://127.0.0.1:8080/newtask", formData);
  const submitForm = () => {
    if (formData.title && formData.body) {
      sendPost();
    } else {

    }
  }
  return (
    <div className="taskform_main">

      <div>
        Title:
        <input
          className={formData.title ? "black-input" : "red-input"}
          type="text"
          value={formData.title}
          name="title"
          onChange={handleChange}
        />
      </div>

      <div>
        Body:
        <input
          className={formData.body ? "black-input" : "red-input"}
          type="text"
          value={formData.body}
          name="body"
          onChange={handleChange}
        />
      </div>
      <button onClick={submitForm}>Submit</button>
    </div>
  )
}


export default TaskForm;
