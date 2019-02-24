import React, { Component } from 'react';

export default class EquipmentListItem extends Component {
  // constructor(props) {
  //   super(props);
  //   this.state = {equipment: this.props.equipment};
  // }

  render() {
    return (<li>{this.props.equipment.info}</li>);
  }
}
