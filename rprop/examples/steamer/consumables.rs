use rprop::propose;

propose!(Fuel);
propose!(Heat);
propose!(Steam);
propose!(FeedWater);

propose!(Seawater);
propose!(Coolant = Seawater);
