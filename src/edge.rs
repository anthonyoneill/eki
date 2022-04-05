use crate::node::Node;
use crate::edges::{
    pipe::Pipe,
    valve::Valve,
};
use crate::fluid::Fluid;

#[derive(Clone, PartialEq, Debug, serde::Deserialize, serde::Serialize)]
pub enum Edge {
    Pipe(Pipe),    
    Valve(Valve),   
}

impl std::fmt::Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Edge::Pipe(_edge) => write!(f, "Pipe"),
            Edge::Valve(_edge) => write!(f, "Valve"),
        }
    }
}

impl Edge {
    pub fn from(&self) -> Node {
        match self {
            Edge::Pipe(edge) => edge.from.clone(),
            Edge::Valve(edge) => edge.from.clone(),
        }
    }

    pub fn to(&self) -> Node {
        match self {
            Edge::Pipe(edge) => edge.to.clone(),
            Edge::Valve(edge) => edge.to.clone(),
        }
    }

    pub fn id(&self) -> (usize, usize) {
        match self {
            Edge::Pipe(edge) => (edge.from.id(), edge.to.id()),
            Edge::Valve(edge) => (edge.from.id(), edge.to.id()),
        }
    }

    pub fn mass_flow(&mut self) -> &mut f64 {
        match self {
            Edge::Pipe(edge) => &mut edge.mass_flow,
            Edge::Valve(edge) => &mut edge.mass_flow,
        }
    }

    pub fn length(&mut self) -> Option<&mut f64> {
        match self {
            Edge::Pipe(edge) => Some(&mut edge.length),
            Edge::Valve(_edge) => None,
        }
    }

    pub fn diameter(&mut self) -> &mut f64 {
        match self {
            Edge::Pipe(edge) => &mut edge.diameter,
            Edge::Valve(edge) => &mut edge.diameter,
        }
    }

    pub fn area(&self) -> f64 {
        match self {
            Edge::Pipe(edge) => edge.area(),
            Edge::Valve(edge) => edge.area(),
        }
    }

    pub fn roughness(&mut self) -> Option<&mut f64> {
        match self {
            Edge::Pipe(edge) => Some(&mut edge.roughness),
            Edge::Valve(_edge) => None,
        }
    }

    pub fn thickness(&mut self) -> &mut f64 {
        match self {
            Edge::Pipe(edge) => &mut edge.thickness,
            Edge::Valve(edge) => &mut edge.thickness,
        }
    }

    pub fn youngs_modulus(&mut self) -> &mut f64 {
        match self {
            Edge::Pipe(edge) => &mut edge.youngs_modulus,
            Edge::Valve(edge) => &mut edge.youngs_modulus,
        }
    }

    pub fn open_percent(&mut self) -> Option<&mut f64> {
        match self {
            Edge::Pipe(_edge) => None,
            Edge::Valve(edge) => Some(&mut edge.open_percent),
        }
    }

    pub fn pressure_loss_coefficient(&self) -> Option<f64> {
        match self {
            Edge::Pipe(_edge) => None,
            Edge::Valve(edge) => Some( edge.k() ),
        }
    }

    pub fn wave_speed(&self, fluid: &Fluid) -> f64 {
        match self {
            Edge::Pipe(edge) => edge.wave_speed( fluid ),
            Edge::Valve(edge) => edge.wave_speed( fluid ),
        }
    }

    pub fn r_drdq(&self, flow_rate: f64, nu: f64, g: f64 ) -> (f64, f64) {
        let r = self.resistance( flow_rate, nu, g );
        if flow_rate == 0.0 {
            ( r, 0.0 )
        } else {
            let delta = 1.0e-8;
            let flow_rate_plus = flow_rate + delta;
            let r_plus = self.resistance( flow_rate_plus, nu, g );
            let drdq = ( r_plus - r ) / delta;
            ( r, drdq )
        }
    }

    pub fn resistance(&self, flow_rate: f64, nu: f64, g: f64 ) -> f64 {
        match self {
            Edge::Pipe(edge) => edge.resistance( flow_rate, nu, g ),
            Edge::Valve(edge) => edge.resistance( flow_rate, nu, g ),
        }
    }

    pub fn k_laminar(&self, nu: f64 ) -> f64 {
        match self {
            Edge::Pipe(edge) => edge.k_laminar(nu),
            Edge::Valve(edge) => edge.k_laminar(nu),
        }
    }

    pub fn darcy_approx(&self, head_loss: f64, g: f64 ) -> f64 {
        match self {
            Edge::Pipe(edge) => edge.darcy_approx(head_loss, g),
            Edge::Valve(edge) => edge.darcy_approx(head_loss, g),
        }
    }

    /*pub fn update_from(&mut self, node: Node ) {
        match self {
            Edge::Pipe(edge) => edge.from = node,
            Edge::Valve(edge) => edge.from = node,
        }
    }

    pub fn update_to(&mut self, node: Node ) {
        match self {
            Edge::Pipe(edge) => edge.to = node,
            Edge::Valve(edge) => edge.to = node,
        }
    }*/
}