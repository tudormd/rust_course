mod greeting;
mod goodbye;


fn main() {
    greeting::description(); // greeting message

    greeting::formal::english();
    greeting::formal::spanish();

    greeting::casual::english();
    greeting::casual::spanish();


    goodbye::description(); // goodbye message

    goodbye::formal::english();
    goodbye::formal::spanish();

    goodbye::casual::english();
    goodbye::casual::spanish();

}
