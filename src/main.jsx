import React, { Component } from "react";
import {
  Route,
  Link,
  HashRouter
} from "react-router-dom"

import Home from "./links/home"
import Stuff from "./links/stuff"
import Contact from "./links/contact"
import Graph from "./components/graph"


class Main extends Component {
  render() {
    return (
      <HashRouter>
        <div>
          <h1>Simple SPA</h1>
          <ul className="header">
            <li><Link to="/">Home</Link></li>
            <li><Link to="/stuff">Stuff</Link></li>
            <li><Link to="/contact">Contact</Link></li>
            <li><Link to="/graph">Graph</Link></li>
          </ul>
          <div className="content">
             <Route exact path="/" component={Home} />
             <Route path="/stuff" component={Stuff} />
             <Route path="/contact" component={Contact} />
             <Route path="/graph" component={Graph} />
          </div>
        </div>
      </HashRouter>
    );
  }
}
 
export default Main;