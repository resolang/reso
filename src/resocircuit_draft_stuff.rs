

// 1. ResoCircuit struct, which is instantiated from a ReselBoard
// 2. Simple mappings for region indices. (class_by_region, wire_nodes, etc.)
// 3. With that done, adjacency mappings. (input_to_wire, etc.)
// 4. Iteration loop. (see readme)
// 5. Serialization, etc?
impl ResoCircuit {
  fn from_image(img: &DynamicImage) -> Self {
    let reselboard = image_to_reselboard(img);
    let (width, height) = (reselboard.len(), reselboard[0].len());

    let (region_by_resel, resels_by_region) = resel_region_mapping_from_reselboard(
      &reselboard
    );

    let (
      class_by_region, wire_nodes, input_nodes, output_nodes, logic_nodes
    ) = class_indices_from_reselboard_and_regions(
      &reselboard, &region_by_resel, &resels_by_region
    );

    // TODO from here down

    // map input_idx to adjacent regions
    let input_adjacencies: Vec<Vec<usize>> = input_nodes.iter().map(
      |&input_region_idx| {
        get_adjacent_region_idxs(input_region_idx, &region_by_resel, &resels_by_region)
      }
    ).collect();

    /*
    input_idx_to_output_idx: Vec<Vec<usize>> = empty;

    for each (input_local_ii, input_region_ii):
      get adjacencies for input_region_ii
      for each adjacent region,
        check the class.
        if it is an output node, we want to add it to input_idx_to_output_idx
        note: we want to add the local index, not the region index
        (we will need to find it thru `output_nodes`)
      ( O(n) searchup could be made O(logn) by assuming 
        class_indices_from_reselboard_and_regions is sorted first
      )
    */
    let input_to_output = 

    /*
    let input_to_wire = input_adjacencies.iter().filter(
      |adj_region_idx| {
        vec![
          Resel::WireOrangeOff, Resel::WireOrangeOn,
          Resel::WireSapphireOff, Resel::WireSapphireOn,
          Resel::WireLimeOff, Resel::WireLimeOn,
        ].contains(&class_by_region[adj_region_idx])
      }
    ).collect();

    let input_to_logic = &input_adjacencies.iter().filter(
      |adj_region_idx| {
        vec![Resel::AND, Resel::XOR].contains(&class_by_region[adj_region_idx])
      }
    ).collect();

    let input_to_output = &input_adjacencies.iter().filter(
      |&adj_region_idx| {
        &class_by_region[*adj_region_idx] == Resel::Output;
      }
    ).collect();
    */


    let logic_to_output = vec![vec![0; 0]; 0];
    let output_to_wire = vec![vec![0; 0]; 0];
    let wire_state = vec![false; 0];
    let logic_state = vec![false; 0];
    let output_state = vec![false; 0];

    ResoCircuit {
      image: img.clone(),
      reselboard: reselboard,
      region_by_resel: region_by_resel,
      resels_by_region: resels_by_region,
      class_by_region: class_by_region,
      wire_nodes: wire_nodes,
      input_nodes: input_nodes,
      output_nodes: output_nodes,
      logic_nodes: logic_nodes,
      input_to_wire: input_to_wire,
      input_to_logic: input_to_logic,
      input_to_output: input_to_output,
      logic_to_output: logic_to_output,
      output_to_wire: output_to_wire,
      wire_state: wire_state,
      logic_state: logic_state,
      output_state: output_state,
    }
  }



  //fn from_reselboard(&self, reselboard: &Vec<Vec<Resel>) -> Self {
  //  let (width, height) = (reselboard.len(), reselboard[0].len());
  //}

  /*
  fn from_image(img: &image::DynamicImage) -> Self {
    let reselboard = image_to_reselboard(img);
    Self::from_reselboard(&reselboard)
  }

  fn from_filename(filename: &str) -> Self {
    let img = load_image_from_filename(filename);
    Self::from_image(&img)
  }
  */
}