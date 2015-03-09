{"filter":false,"title":"circle.rs","tooltip":"/method_syntax/src/shapes/circle.rs","undoManager":{"mark":22,"position":22,"stack":[[{"group":"doc","deltas":[{"start":{"row":0,"column":0},"end":{"row":53,"column":1},"action":"insert","lines":["#![feature(core)]","","use std::f64::consts;","","struct Circle {","    x: f64,","    y: f64,","    radius: f64,","}","","impl Circle {","    pub fn new(x: f64, y: f64, radius: f64) -> Circle {","        Circle {","            x: x,","            y: y,","            radius: radius,","        }","    }","","    pub fn area(&self) -> f64 {","        consts::PI * (self.radius * self.radius)","    }","    ","    pub fn grow(&self) -> Circle {","        Circle { x: self.x, y: self.y, radius: (self.radius * 10.0) }","    }","}","","struct CircleBuilder {","    coordinate_x: f64,","    coordinate_y: f64,","    radius: f64,","}","","impl CircleBuilder {","    pub fn new() -> CircleBuilder {","        CircleBuilder { coordinate_x: 0.0, coordinate_y: 0.0, radius: 1.0 }","    }","    ","    pub fn coordinate(&mut self, coordinate_x: f64, coordinate_y: f64) -> &mut CircleBuilder {","        self.coordinate_x = coordinate_x;","        self.coordinate_y = coordinate_y;","        self","    }","    ","    pub fn radius(&mut self, radius: f64) -> &mut CircleBuilder {","        self.radius = radius;","        self","    }","    ","    pub fn finalize(&self) -> Circle {","        Circle { x: self.coordinate_x, y: self.coordinate_y, radius: self.radius }","    }","}"]}]}],[{"group":"doc","deltas":[{"start":{"row":4,"column":0},"end":{"row":4,"column":1},"action":"insert","lines":["p"]}]}],[{"group":"doc","deltas":[{"start":{"row":4,"column":1},"end":{"row":4,"column":2},"action":"insert","lines":["u"]}]}],[{"group":"doc","deltas":[{"start":{"row":4,"column":2},"end":{"row":4,"column":3},"action":"insert","lines":["b"]}]}],[{"group":"doc","deltas":[{"start":{"row":4,"column":3},"end":{"row":4,"column":4},"action":"insert","lines":[" "]}]}],[{"group":"doc","deltas":[{"start":{"row":28,"column":0},"end":{"row":28,"column":1},"action":"insert","lines":["p"]}]}],[{"group":"doc","deltas":[{"start":{"row":28,"column":1},"end":{"row":28,"column":2},"action":"insert","lines":["u"]}]}],[{"group":"doc","deltas":[{"start":{"row":28,"column":2},"end":{"row":28,"column":3},"action":"insert","lines":["b"]}]}],[{"group":"doc","deltas":[{"start":{"row":28,"column":3},"end":{"row":28,"column":4},"action":"insert","lines":[" "]}]}],[{"group":"doc","deltas":[{"start":{"row":5,"column":4},"end":{"row":5,"column":5},"action":"insert","lines":["p"]}]}],[{"group":"doc","deltas":[{"start":{"row":5,"column":5},"end":{"row":5,"column":6},"action":"insert","lines":["u"]}]}],[{"group":"doc","deltas":[{"start":{"row":5,"column":6},"end":{"row":5,"column":7},"action":"insert","lines":["b"]}]}],[{"group":"doc","deltas":[{"start":{"row":5,"column":7},"end":{"row":5,"column":8},"action":"insert","lines":[" "]}]}],[{"group":"doc","deltas":[{"start":{"row":6,"column":4},"end":{"row":6,"column":8},"action":"insert","lines":["pub "]}]}],[{"group":"doc","deltas":[{"start":{"row":7,"column":4},"end":{"row":7,"column":8},"action":"insert","lines":["pub "]}]}],[{"group":"doc","deltas":[{"start":{"row":29,"column":4},"end":{"row":29,"column":8},"action":"insert","lines":["pub "]}]}],[{"group":"doc","deltas":[{"start":{"row":30,"column":4},"end":{"row":30,"column":8},"action":"insert","lines":["pub "]}]}],[{"group":"doc","deltas":[{"start":{"row":31,"column":4},"end":{"row":31,"column":8},"action":"insert","lines":["pub "]}]}],[{"group":"doc","deltas":[{"start":{"row":0,"column":0},"end":{"row":0,"column":17},"action":"remove","lines":["#![feature(core)]"]}]}],[{"group":"doc","deltas":[{"start":{"row":0,"column":0},"end":{"row":0,"column":17},"action":"insert","lines":["#![feature(core)]"]}]}],[{"group":"doc","deltas":[{"start":{"row":0,"column":0},"end":{"row":0,"column":17},"action":"remove","lines":["#![feature(core)]"]}]}],[{"group":"doc","deltas":[{"start":{"row":1,"column":0},"end":{"row":2,"column":0},"action":"remove","lines":["",""]}]}],[{"group":"doc","deltas":[{"start":{"row":0,"column":0},"end":{"row":1,"column":0},"action":"remove","lines":["",""]}]}]]},"ace":{"folds":[],"scrolltop":0,"scrollleft":0,"selection":{"start":{"row":8,"column":13},"end":{"row":8,"column":13},"isBackwards":false},"options":{"guessTabSize":true,"useWrapMode":false,"wrapToView":true},"firstLineState":0},"timestamp":1425933770942,"hash":"7aaa99bb562c777299d3d087956011a57e1108e3"}