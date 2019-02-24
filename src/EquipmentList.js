import React, { Component } from 'react';
import EquipmentListItem from './EquipmentListItem';

export default class EquipmentList extends Component {
  render() {
    let eli = this.props.equipment.map(elem => {
      return <EquipmentListItem key={elem._id} equipment={elem}></EquipmentListItem>;
    });
    return (<ul>{eli}</ul>);
  }
}
