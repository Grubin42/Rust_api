document.addEventListener('DOMContentLoaded', () => {
    fetch('http://localhost:8000/api/users')
        .then(response => {
            if (!response.ok) {
                throw new Error('Network response was not ok');
            }
            return response.json();
        })
        .then(users => {
            const usersList = document.getElementById('users-list');
            users.forEach(user => {
                const li = document.createElement('li');
                li.textContent = `${user.name} (${user.email})`;
                usersList.appendChild(li);
            });
        })
        .catch(error => console.error('Error fetching users:', error));
});