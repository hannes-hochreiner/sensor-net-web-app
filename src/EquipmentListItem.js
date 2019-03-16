import React, { Component } from 'react';
import { Card, CardHeader, CardText, CardBody, CardTitle, CardSubtitle, Button } from 'reactstrap';

export default class EquipmentListItem extends Component {
  // constructor(props) {
  //   super(props);
  //   this.state = {equipment: this.props.equipment};
  // }

  render() {
    let pars = [];

    for (let elem in this.props.equipment.parameters) {
      let par = this.props.equipment.parameters[elem][0];
      let parType = this.props.parameter_types.find(elem => {
        return elem._id === par._parameter_type_id;
      });

      pars.push(`${parType.id}: ${par.val.toFixed(1)} ${parType.unit}`);
    }

    return (<div>
      <Card>
        <CardHeader>{this.props.equipment.info}</CardHeader>
        <CardBody>
          <CardText>
            {pars.join('; ')}
          </CardText>
        </CardBody>
      </Card>
    </div>);
  }
}
