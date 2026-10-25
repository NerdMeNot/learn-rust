using System;

var line = "mode";
var parts = line.Split('=');
Console.WriteLine($"{parts[0]} -> {parts[1]}");
