use rprop::propose;

fn main() {
    propose!(TapWater);
    propose!(BottledWater);

    propose!(HasWater = TapWater || BottledWater);
    propose!(Kettle);
    propose!(BoiledWater = Kettle && HasWater);

    propose!(Teabag);
    propose!(Cup);
    propose!(Tea);

    propose!(MakeTea = Teabag && Cup && BoiledWater -> Tea);
}