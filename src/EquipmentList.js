import React, { Component } from 'react';
import EquipmentListItem from './EquipmentListItem';
import { Container, Row, Col } from 'reactstrap';

export default class EquipmentList extends Component {
  render() {
    console.log(this.props.equipment);
    console.log(this.props.parameter_types);
    let eli = this.props.equipment.map(elem => {
      return <Row key={elem._id}><Col><EquipmentListItem key={elem._id} equipment={elem} parameter_types={this.props.parameter_types}></EquipmentListItem></Col></Row>;
    });
    return (<Container>{eli}</Container>);
  }
}
