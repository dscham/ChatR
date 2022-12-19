const invoke = window.__TAURI__.invoke;

let logged_in_title = () => document.getElementById('logged_in');
let logged_out_title = () => document.getElementById('logged_out');

let username_output = () => document.getElementById('current_username');
let username_input = () => document.getElementById('new_username');

function changeUsername() {
    invoke('save_username', {username: username_input().value})
        .then(host_info => {
            showUsername(host_info.name)
        })
        .catch(err => console.log(err));

}

function showUsername(username) {
    username_output().textContent = username;
    logged_out_title().style.display = 'none';
    logged_in_title().style.display = 'block';
}

document.addEventListener('DOMContentLoaded', () => {
    logged_out_title().style.display = 'block';

    invoke('get_host_info')
        .then(host_info => {
            console.log(host_info);
            showUsername(host_info.name);
        })
        .catch(err => console.log(err));
});