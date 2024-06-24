async function loadProfile() {
    const response = await fetch('http://localhost:8000/api/register', {
        method: 'GET',
        headers: {
            'Authorization': 'Bearer token', // Replace with your token logic
        },
    });

    if (response.ok) {
        const user = await response.json();
        document.getElementById('profile-info').innerText = `Name: ${user.name}\nEmail: ${user.email}`;
    } else {
        alert('Failed to load profile.');
    }
}

document.getElementById('edit-profile').addEventListener('click', function() {
    window.location.href = '/html/edit_profile.html';
});

document.getElementById('delete-profile').addEventListener('click', async function() {
    const response = await fetch('/api/profile/delete', {
        method: 'DELETE',
        headers: {
            'Authorization': 'Bearer token', // Replace with your token logic
        },
    });

    if (response.ok) {
        alert('Profile deleted.');
        window.location.href = '/html/register.html';
    } else {
        alert('Failed to delete profile.');
    }
});

loadProfile();