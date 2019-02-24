import React, { Component } from 'react';
import logo from './logo.svg';
import './App.css';
import { default as axios } from 'axios';
import Repo from './Repo.mock';
import EquipmentList from './EquipmentList';

class App extends Component {
  constructor(props) {
    super(props);
    this.state = {
      equipment: [],
      parameter_types: []
    };
    this.repo = new Repo();
  }

  componentDidMount() {
    (async () => {
      let equipment = await this.repo.getEquipment();
      let parameter_types = await this.repo.getParameterTypes();
      let ts_start = new Date();
      ts_start.setMinutes(ts_start.getMinutes() - 2);
      let parameter = await this.repo.getParameters(ts_start, new Date());
      let param_equip_dict = parameter.reduce((acc, curr) => {
        if (!acc.hasOwnProperty(curr._equipment_id)) {
          acc[curr._equipment_id] = [];
        }

        acc[curr._equipment_id].push(curr);

        return acc;
      }, {});

      equipment.forEach(elem => {
        if (param_equip_dict.hasOwnProperty(elem._id)) {
          elem.parameters = param_equip_dict[elem._id];
          elem.parameters.sort((a, b) => {
            if (a < b) {
              return -1;
            }

            if (a > b) {
              return 1;
            }

            return 0;
          });
        }
      });

      this.setState({
        equipment: equipment,
        parameter_types: parameter_types
      });
    })();
  }

  render() {
    return (
      <div className="App">
        <header className="App-header">
          <EquipmentList equipment={this.state.equipment}></EquipmentList>
        </header>
      </div>
    );
  }
}

export default App;
