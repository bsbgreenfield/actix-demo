import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import TaskForm from './TaskForm.tsx'
import ShowTasks from './ShowTasks.tsx'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <div className='wrapper'>
      <TaskForm />
      <ShowTasks />
    </div>
  </StrictMode >,
)
