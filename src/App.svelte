<script lang="ts">
  import {
    Button,
    Card,
    CardBody,
    CardHeader,
    CardText,
    CardTitle,
    Container,
    Collapse,
    Input,
    FormGroup,
    Label,
    Row,
    Col,
  } from "sveltestrap";
  let isOpen = false;
  import { invoke } from "@tauri-apps/api/tauri";
  import { Styles } from "sveltestrap";

  let input_ph: number = 7;
  let converted_ph: number = 0;
  let calib: Array<number> = [1.0, 0.0];

  const handleClick = async () => {
    converted_ph = await invoke("convert", {
      ph: input_ph,
      slope: calib[0],
      offset: calib[1],
    });
  };

  let ph4: number = 4.01;
  let ph10: number = 10.02;
  let temperature: number = 21.0;
  const handleCalib = async () => {
    calib = await invoke("calibrate", {
      ph4: ph4,
      ph10: ph10,
      temperature: temperature,
    });
  };
</script>

<Styles />

<main>
  <h1>Caliph</h1>
  <Container>
    <Button color="secondary" on:click={() => (isOpen = !isOpen)} class="mb-3">
      Calibrate pH
    </Button>
    <Row cols={2}>
      <Col>
        <Collapse {isOpen}>
          <Card>
            <CardHeader>
              <CardTitle>Measured pH</CardTitle>
            </CardHeader>
            <CardBody>
              <FormGroup>
                <Label for="ph4Input">ph 4:</Label>
                <Input
                  type="number"
                  name="ph4Input"
                  id="ph4Input"
                  placeholder="ph 4"
                  bind:value={ph4}
                  min={-5}
                  max={15}
                  step={0.01}
                />
                <Label for="ph10Input">ph 10:</Label>
                <Input
                  type="number"
                  name="ph10Input"
                  id="ph10Input"
                  placeholder="ph 10"
                  bind:value={ph10}
                  min={-5}
                  max={15}
                  step={0.01}
                />
                <Label for="tempInput">Temperature:</Label>
                <Input
                  type="number"
                  name="temperature"
                  id="tempInput"
                  placeholder="temperature"
                  bind:value={temperature}
                  min={-20}
                  max={120}
                  step={0.1}
                /></FormGroup
              >
              <Button color="primary" on:click={handleCalib}>Calibrate</Button>
            </CardBody>
          </Card>
        </Collapse></Col
      >
      <Col>
        <Collapse {isOpen}>
          <Card>
            <CardHeader><CardTitle>Constants</CardTitle></CardHeader>
            <CardBody
              ><FormGroup>
                <Label for="inputSlope">Slope:</Label>
                <Input
                  type="number"
                  name="slope"
                  id="inputSlope"
                  placeholder="slope"
                  bind:value={calib[0]}
                  step={0.0001}
                />
                <Label for="inputOffset">Offset:</Label>
                <Input
                  type="number"
                  name="slope"
                  id="inputOffset"
                  placeholder="offset"
                  bind:value={calib[1]}
                  step={0.0001}
                /></FormGroup
              ></CardBody
            >
          </Card>
        </Collapse>
      </Col>
    </Row>
  </Container>

  <Container>
    <Card class="mb-3">
      <CardHeader>
        <CardTitle>Convert pH</CardTitle>
      </CardHeader>
      <CardBody>
        <Row cols="2">
          <Col>
            <CardText>Input pH:</CardText>
            <Input
              type="number"
              id="exampleRange"
              placeholder="number placeholder"
              bind:value={input_ph}
              step={0.01}
            />
          </Col>

          <Col>
            <Button color="primary" on:click={handleClick}>Convert</Button>
            <CardTitle>{converted_ph.toFixed(2)}</CardTitle>
          </Col>
        </Row>
      </CardBody>
    </Card>
  </Container>
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 420px;
    margin: 0 auto;
  }

  h1 {
    text-align: center;
    color: #ff3e00;
    text-transform: uppercase;
    font-size: 4em;
    font-weight: 100;
    padding: 0;
  }
</style>
