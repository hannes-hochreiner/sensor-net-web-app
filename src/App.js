import React, { Component } from 'react';
import logo from './logo.svg';
// import './App.css';
import { default as axios } from 'axios';
import Repo from './Repo.mock';
import { TabContent, TabPane, Nav, NavItem, NavLink } from 'reactstrap';
import classnames from 'classnames';
import EquipmentList from './EquipmentList';

class App extends Component {
  constructor(props) {
    super(props);
    this.state = {
      equipment: [],
      parameter_types: [],
      activeTab: '1'
    };
    this.switchTab = this.switchTab.bind(this);
    this.repo = new Repo(axios);
  }

  switchTab(tab) {
    if (this.state.activeTab !== tab) {
      this.setState({activeTab: tab});
    }
  }

  componentDidMount() {
    (async () => {
      let equipment = await this.repo.getEquipment();
      let parameter_types = await this.repo.getParameterTypes();
      let ts_start = new Date();
      ts_start.setMinutes(ts_start.getMinutes() - 20);
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
          elem.parameters = param_equip_dict[elem._id].reduce((prev, curr) => {
            if (!prev.hasOwnProperty(curr._parameter_type_id)) {
              prev[curr._parameter_type_id] = [];
            }

            prev[curr._parameter_type_id].push(curr);

            return prev;
          }, {});

          for (let par in elem.parameters) {
            // desc
            elem.parameters[par].sort((a, b) => {
              if (a.ts < b.ts) {
                return 1;
              }
  
              if (a.ts > b.ts) {
                return -1;
              }
  
              return 0;
            });
          }
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
        <Nav tabs>
          <NavItem>
            <NavLink
              className={classnames({ active: this.state.activeTab === '1' })}
              onClick={() => { this.switchTab('1'); }}
            >
              Values
            </NavLink>
          </NavItem>
          <NavItem>
            <NavLink
              className={classnames({ active: this.state.activeTab === '2' })}
              onClick={() => { this.switchTab('2'); }}
            >
              Graphs
            </NavLink>
          </NavItem>
        </Nav>
        <TabContent activeTab={this.state.activeTab}>
          <TabPane tabId="1">
            <EquipmentList equipment={this.state.equipment} parameter_types={this.state.parameter_types}></EquipmentList>
          </TabPane>
          <TabPane tabId="2">
            empty
          </TabPane>
        </TabContent>
      </div>
    );
  }
}

export default App;
