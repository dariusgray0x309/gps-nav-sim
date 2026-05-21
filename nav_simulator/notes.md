path forward (as of 5/20/26) create way-points for the vehicle to follow 
- Likely can use an array of points and update car.simulate_motion() to go through each point until the end goal is met

After that, create different executables (bin directory already exists)

Next, generate python code to plot
- Not sure if it makes sense to try to stream things live to Python or just store output files that Python can parse

Then incorporate the satellite code into the vehicle, likely:
- car.position(Satellite::compute_trilateration(&sat1, &sat2, &sat3));
- let rel_pos = self.compute_relative_position(goal); // this already exists
    - This will also include networking (ZMQ pub/sub)
