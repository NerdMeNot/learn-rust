interface Speaker {
  speak(): string;
}
class Robot {
  speak() {
    return "beep";
  }
}
function say(s: Speaker) {
  console.log(s.speak());
}
say(new Robot()); // Robot never mentions Speaker, and it fits
say({ speak: () => "hi" }); // so does an object literal
