const invoke = window.__TAURI__.invoke;

let logged_in_title = () => document.getElementById('logged_in');
let logged_out_title = () => document.getElementById('logged_out');

let username_output = () => document.getElementById('current_username');
let username_input = () => document.getElementById('new_username');

document.addEventListener('DOMContentLoaded', () => {
    updateUsernameOutput();
});

function changeUsername() {
    invoke('save_username', {username: username_input().value})
        .then(host_info => {
            updateUsernameOutput(host_info.name)
        })
        .catch(err => console.log(err));

}

function updateUsernameOutput(username = 'unknown') {
    username_output().textContent = username;
    if(isFirstRun()) {
        logged_out_title().style.display = 'block';
        logged_in_title().style.display = 'none';
    } else {
        logged_in_title().style.display = 'none';
        logged_out_title().style.display = 'block';
    }
}

function isFirstRun() {
    return invoke('is_first_run').then(result => {return result});
}