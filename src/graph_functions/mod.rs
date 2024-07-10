use std::collections::VecDeque;

use bevy::{ecs::{query::{QueryFilter, QueryEntityError}, entity::Entity, system::Query, component::Component}, utils::hashbrown::HashSet};


#[derive(Debug)]
pub enum GraphError{
    NoPath,
    InvalidEntity,
    NegativeWeight
}

impl From<QueryEntityError> for GraphError {
    fn from(_: QueryEntityError) -> Self {
        Self::InvalidEntity
    }
}

pub trait GraphVertex : Component{
    fn get_neighbours(&self) -> Vec<Entity>;
}


pub fn within_steps<V:GraphVertex, T:QueryFilter>(
    start_ent: Entity,
    max_steps: usize,
    query: &Query<&V, T>
) -> Result<Vec<(Entity, usize)>, GraphError> {

    //vector of vertices that we want to check, alongside their distance from the start vertex
    let mut to_view: VecDeque<&V> = VecDeque::from([query.get(start_ent)?]);

    //hashset storing entities weve checked already
    let mut seen: HashSet<Entity> = HashSet::new();
    seen.insert(start_ent);

    //final output list
    let mut valid: Vec<(Entity, usize)> = vec![(start_ent, 0)];

    //the current step we are on
    let mut current_step = 0;
    //the number of vertices left to check at this distance
    let mut at_current_step = 1;
    
    while let Some(current_vert) = to_view.pop_front(){
        
        //decrement number left to check at this distance
        at_current_step -= 1;

        for neighbour in current_vert.get_neighbours(){
            //check if we have checked this entity before, skipping this iteration if so
            if !seen.insert(neighbour){continue;}
            //otherwise add it to the valid list and to_view queue
            if let Ok(vert) = query.get(neighbour){
                to_view.push_back(vert);
                valid.push((neighbour, current_step+1));
            }
        }

        //if we've viewed all at the current step, increment current_step and calculate how many at this step
        if at_current_step == 0 {
            current_step += 1;
            //check if we've checked far enough
            if current_step == max_steps {break;}
            at_current_step = to_view.len(); //if this is 0, we shouldnt run another loop iteration so should be ok
        }
    }
    Ok(valid)
}
