document.getElementById('edit-profile-form').addEventListener('submit', async function(event) {
    event.preventDefault();
    const name = document.getElementById('name').value;
    const email = document.getElementById('email').value;

    const response = await fetch('http://localhost:8000/api/register', {
        method: 'PUT',
        headers: {
            'Content-Type': 'application/x-www-form-urlencoded',
            'Authorization': 'Bearer token', // Replace with your token logic
        },
        body: `name=${encodeURIComponent(name)}&email=${encodeURIComponent(email)}`,
    });

    if (response.ok) {
        alert('Profile updated.');
        window.location.href = '/html/profile.html';
    } else {
        alert('Failed to update profile.');
    }
});

async function loadProfile() {
    const response = await fetch('/api/profile', {
        method: 'GET',
        headers: {
            'Authorization': 'Bearer token', // Replace with your token logic
        },
    });

    if (response.ok) {
        const user = await response.json();
        document.getElementById('name').value = user.name;
        document.getElementById('email').value = user.email;
    } else {
        alert('Failed to load profile.');
    }
}

loadProfile();