import { invoke } from '@tauri-apps/api/tauri';

function fetchTasks() {
  invoke('fetch_tasks')
    .then((tasks: any) => {
      const taskList = document.getElementById('taskList') as any;

      taskList.innerHTML = '';

      tasks.forEach((task: any, index: any) => {
        const listItem = document.createElement('li');
        listItem.className =
          'list-group-item d-flex justify-content-between align-items-center';
        listItem.textContent = task;

        const buttonsDiv = document.createElement('div');

        const updateButton = document.createElement('button');
        updateButton.className = 'btn btn-info btn-sm';
        updateButton.textContent = 'Update';
        updateButton.onclick = function () {
          updateTask(index);
        };

        const deleteButton = document.createElement('button');
        deleteButton.className = 'btn btn-danger btn-sm';
        deleteButton.textContent = 'Delete';
        deleteButton.onclick = function () {
          deleteTask(index);
        };

        buttonsDiv.appendChild(updateButton);
        buttonsDiv.appendChild(deleteButton);

        listItem.appendChild(buttonsDiv);
        taskList.appendChild(listItem);
      });
    })
    .catch((error) => {
      console.error('Error:', error);
      alert('An error occurred while fetching tasks.');
    });
}

function addTask() {
  const taskInput = document.getElementById('taskInput') as any;
  const task = taskInput.value.trim();

  if (task !== '') {
    invoke('add_task', { task })
      .then(() => {
        fetchTasks();
        taskInput.value = '';
      })
      .catch((error: any) => {
        console.log(error);
      });
  } else {
    alert('Please enter a task.');
  }
}

function updateTask(index: any) {
  const updatedTask = prompt('Enter updated task:');
  if (updatedTask !== null) {
    invoke('update_task', { index, updatedTask })
      .then(() => {
        fetchTasks();
      })
      .catch((error) => {
        console.error('Error:', error);
        alert('An error occurred.');
      });
  }
}

function deleteTask(index: any) {
  const confirmation = confirm('Are you sure you want to delete this task?');
  if (confirmation) {
    invoke('delete_task', { index })
      .then(() => {
        fetchTasks();
      })
      .catch((error) => {
        console.error('Error:', error);
        alert('An error occurred.');
      });
  }
}

function attachEventListeners() {
  const addButton = document.querySelector('.btn-primary');
  if (addButton) {
    addButton.addEventListener('click', addTask);
  } else {
    console.error('Button not found!');
  }
}

fetchTasks();
attachEventListeners();
