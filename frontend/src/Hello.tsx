import React, { useEffect, useState } from "react";
import axios from "axios";


const Hello = () => {
  const [message, setMessage] = useState<string>();

  useEffect(() => {
    axios.get("http://127.0.0.1:8080/")
      .then(response => {
        console.log(response);
        setMessage(response.data);
      }

      )
      .catch(error => console.log(error));
  }, [])

  return (
    <h1>{message}!</h1>
  )
}

export default Hello;
